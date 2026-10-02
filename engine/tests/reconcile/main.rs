//! The `reconcile` module held to the Python engine it ports: whole
//! recorded runs replayed, the function cases of `main.py`, and the
//! reconciliation tests of `pr_review/tests` as scenarios on a stateful
//! fake of GitHub.

mod checklist;
mod fake;
mod functions;
mod properties;
mod publication;
mod recordings;
mod scene;
mod support;
mod writes;
