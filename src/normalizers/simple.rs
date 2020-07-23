use crate::model::election::{Ballot, Choice};
use std::collections::BTreeSet;

pub fn simple_normalizer(ballot: Ballot) -> Ballot {
    let mut seen = BTreeSet::new();
    let Ballot { id, choices } = ballot;
    let mut new_choices = Vec::new();
    let mut undervote = false;

    for choice in choices {
        match choice {
            Choice::Vote(v) => {
                if seen.contains(&v) {
                    undervote = true;
                } else {
                    seen.insert(v);
                    new_choices.push(Choice::Vote(v));
                }
            }
            Choice::Undervote => {
                undervote = true;
            }
            Choice::Overvote => {
                new_choices.push(Choice::Overvote);
                undervote = false;
                break;
            }
        }
    }

    if undervote {
        new_choices.push(Choice::Undervote)
    }

    Ballot {
        id,
        choices: new_choices,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::election::{CandidateId, Choice};

    #[test]
    fn test_pass_through() {
        let c1 = Choice::Vote(CandidateId(1));
        let c2 = Choice::Vote(CandidateId(2));
        let c3 = Choice::Vote(CandidateId(3));
        let b = Ballot::new("1".into(), vec![c1, c2, c3]);

        assert_eq!(
            Ballot::new("1".into(), vec![c1, c2, c3]),
            simple_normalizer(b)
        );
    }

    #[test]
    fn test_remove_duplicate() {
        let c1 = Choice::Vote(CandidateId(1));
        let c2 = Choice::Vote(CandidateId(2));
        let b = Ballot::new("1".into(), vec![c1, c2, c1]);

        assert_eq!(
            Ballot::new("1".into(), vec![c1, c2, Choice::Undervote]),
            simple_normalizer(b)
        );
    }

    #[test]
    fn test_remove_multiple() {
        let c1 = Choice::Vote(CandidateId(1));
        let b = Ballot::new("1".into(), vec![c1, c1, c1, c1]);

        assert_eq!(
            Ballot::new("1".into(), vec![c1, Choice::Undervote]),
            simple_normalizer(b)
        );
    }

    #[test]
    fn test_undervote() {
        let c1 = Choice::Vote(CandidateId(1));
        let c2 = Choice::Vote(CandidateId(2));
        let b = Ballot::new("1".into(), vec![c1, Choice::Undervote, c2]);

        assert_eq!(
            Ballot::new("1".into(), vec![c1, c2, Choice::Undervote]),
            simple_normalizer(b)
        );
    }

    #[test]
    fn test_overvote() {
        let c1 = Choice::Vote(CandidateId(1));
        let c2 = Choice::Vote(CandidateId(2));
        let b = Ballot::new("1".into(), vec![c1, Choice::Overvote, c2]);

        assert_eq!(
            Ballot::new("1".into(), vec![c1, Choice::Overvote]),
            simple_normalizer(b)
        );
    }
}
