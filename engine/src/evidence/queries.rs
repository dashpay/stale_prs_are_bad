//! The GraphQL documents the reader sends, byte for byte as
//! `pr_review/github.py` writes them, whitespace included. A replay finds
//! its answer by the exact text of the request, and a transport that only
//! lets the engine's own reads through will allow these texts and no
//! others, so not one character may differ.

use crate::pycompat::PyInt;

/// A head's checks: `GitHub.BUILD_QUERY`.
pub const BUILD: &str = concat!(
    "query($owner:String!, $repo:String!, $number:Int!, $cursor:String) {\n",
    "      repository(owner:$owner, name:$repo) { pullRequest(number:$number) {\n",
    "        commits(last:1) { nodes { commit { oid statusCheckRollup {\n",
    "          contexts(first:100, after:$cursor) {\n",
    "            totalCount pageInfo { hasNextPage endCursor }\n",
    "            nodes {\n",
    "              __typename\n",
    "              ... on CheckRun { name conclusion status startedAt detailsUrl\n",
    "                checkSuite { workflowRun { workflow { resourcePath } } } }\n",
    "              ... on StatusContext { context state createdAt }\n",
    "            } } } } } } } } }",
);

/// A pull request's review threads, a page at a time: `GitHub.threads`.
pub const THREADS: &str = concat!(
    "query($owner:String!, $repo:String!, $number:Int!, $cursor:String) {\n",
    "          repository(owner:$owner, name:$repo) {\n",
    "            pullRequest(number:$number) {\n",
    "              reviewThreads(first:100, after:$cursor) {\n",
    "                totalCount\n",
    "                pageInfo { hasNextPage endCursor }\n",
    "                nodes { id isResolved\n",
    "                  opening: comments(first:1) { nodes { body } }\n",
    "                  comments(first:100) { nodes { author { login } createdAt } }\n",
    "                }\n",
    "              }\n",
    "            }\n",
    "          }\n",
    "        }",
);

/// One pull request's comments, a page at a time, each with its editor:
/// `GitHub._comment_pages`.
pub const COMMENT_PAGES: &str = concat!(
    "query($owner:String!, $repo:String!, $number:Int!, $after:String) {\n",
    "          repository(owner:$owner, name:$repo) {\n",
    "            pullRequest(number:$number) {\n",
    "              comments(first:100, after:$after) {\n",
    "                totalCount\n",
    "                pageInfo { hasNextPage endCursor }\n",
    "                nodes { databaseId body createdAt updatedAt lastEditedAt author { login __typename }\n",
    "                        editor { login __typename } }\n",
    "              }\n",
    "            }\n",
    "          }\n",
    "        }",
);

/// What precedes the aliased pull requests in `GitHub.histories`' query.
const HISTORIES_HEAD: &str =
    "query($owner:String!, $repo:String!) { repository(owner:$owner, name:$repo) {";

/// What follows them: the fragment each alias reads.
const HISTORIES_TAIL: &str = concat!(
    " } }\n",
    "        fragment history on PullRequest {\n",
    "          number\n",
    "          comments(last:100) {\n",
    "            totalCount\n",
    "            nodes { databaseId body createdAt updatedAt lastEditedAt author { login __typename }\n",
    "                    editor { login __typename } }\n",
    "          }\n",
    "          timelineItems(last:1, itemTypes:[CLOSED_EVENT, CONVERT_TO_DRAFT_EVENT]) {\n",
    "            nodes {\n",
    "              ... on ClosedEvent { createdAt }\n",
    "              ... on ConvertToDraftEvent { createdAt }\n",
    "            }\n",
    "          }\n",
    "        }",
);

/// The alias a pull request's history is answered under.
pub fn history_alias(number: &PyInt) -> String {
    format!("pr{number}")
}

/// `GitHub.histories`' query for these pull requests, which must already be
/// distinct and in ascending order, as Python's `sorted(set(numbers))` is.
pub fn histories(numbers: &[PyInt]) -> String {
    let selections: Vec<String> = numbers
        .iter()
        .map(|number| {
            format!(
                "{}: pullRequest(number:{number}) {{ ...history }}",
                history_alias(number)
            )
        })
        .collect();
    format!("{HISTORIES_HEAD}{}{HISTORIES_TAIL}", selections.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_batched_query_is_pythons_text() {
        // `GitHub.histories([2, 1])`, captured from Python 3.12.
        let python = "query($owner:String!, $repo:String!) { repository(owner:$owner, name:$repo) {pr1: pullRequest(number:1) { ...history }\npr2: pullRequest(number:2) { ...history } } }\n        fragment history on PullRequest {\n          number\n          comments(last:100) {\n            totalCount\n            nodes { databaseId body createdAt updatedAt lastEditedAt author { login __typename }\n                    editor { login __typename } }\n          }\n          timelineItems(last:1, itemTypes:[CLOSED_EVENT, CONVERT_TO_DRAFT_EVENT]) {\n            nodes {\n              ... on ClosedEvent { createdAt }\n              ... on ConvertToDraftEvent { createdAt }\n            }\n          }\n        }";
        assert_eq!(histories(&[PyInt::from(1), PyInt::from(2)]), python);
    }
}
