use crate::model::election::{Ballot, Choice, NormalizedBallot};
use std::collections::BTreeSet;

pub fn simple_normalizer(ballot: Ballot) -> NormalizedBallot {
    let mut seen = BTreeSet::new();
    let Ballot { id, choices } = ballot;
    let mut new_choices = Vec::new();
    let mut overvoted = false;

    for choice in choices {
        match choice {
            Choice::Vote(v) => {
                if !seen.contains(&v) {
                    seen.insert(v);
                    new_choices.push(v);
                }
            }
            Choice::Overvote => {
                overvoted = true;
                break;
            }
            _ => (),
        }
    }

    NormalizedBallot::new(id, new_choices, overvoted)
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

        let normalized = simple_normalizer(b);
        assert_eq!(
            vec![CandidateId(1), CandidateId(2), CandidateId(3)],
            normalized.choices()
        );
        assert_eq!(false, normalized.overvoted);
        assert_eq!("1", normalized.id);
    }

    #[test]
    fn test_remove_duplicate() {
        let c1 = Choice::Vote(CandidateId(1));
        let c2 = Choice::Vote(CandidateId(2));
        let b = Ballot::new("1".into(), vec![c1, c2, c1]);

        let normalized = simple_normalizer(b);
        assert_eq!(vec![CandidateId(1), CandidateId(2)], normalized.choices());
        assert_eq!(false, normalized.overvoted);
        assert_eq!("1", normalized.id);
    }

    #[test]
    fn test_remove_multiple() {
        let c1 = Choice::Vote(CandidateId(1));
        let b = Ballot::new("1".into(), vec![c1, c1, c1, c1]);

        let normalized = simple_normalizer(b);
        assert_eq!(vec![CandidateId(1)], normalized.choices());
        assert_eq!(false, normalized.overvoted);
        assert_eq!("1", normalized.id);
    }

    #[test]
    fn test_undervote() {
        let c1 = Choice::Vote(CandidateId(1));
        let c2 = Choice::Vote(CandidateId(2));
        let b = Ballot::new("1".into(), vec![c1, Choice::Undervote, c2]);

        let normalized = simple_normalizer(b);
        assert_eq!(vec![CandidateId(1), CandidateId(2)], normalized.choices());
        assert_eq!(false, normalized.overvoted);
        assert_eq!("1", normalized.id);
    }

    #[test]
    fn test_overvote() {
        let c1 = Choice::Vote(CandidateId(1));
        let c2 = Choice::Vote(CandidateId(2));
        let b = Ballot::new("1".into(), vec![c1, Choice::Overvote, c2]);

        let normalized = simple_normalizer(b);
        assert_eq!(vec![CandidateId(1)], normalized.choices());
        assert_eq!(true, normalized.overvoted);
        assert_eq!("1", normalized.id);
    }
}
