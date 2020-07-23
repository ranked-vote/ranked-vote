use crate::model::election::{Ballot, Choice};
use std::collections::BTreeSet;


pub fn maine_normalizer(ballot: Ballot) -> Ballot {
    // "Exhausted ballot" means a ballot that does not rank any continuing candidate,
    // contains an overvote at the highest continuing ranking or contains 2 or more
    // sequential skipped rankings before its highest continuing ranking.
    // [IB 2015, c. 3, §5 (NEW).]

    let mut seen = BTreeSet::new();
    let Ballot { id, choices } = ballot;
    let mut new_choices = Vec::new();
    let mut last_skipped = false;
    
    for choice in choices {
        match choice {
            Choice::Vote(v) => {
                if !seen.contains(&v) {
                    seen.insert(v);
                    new_choices.push(Choice::Vote(v));
                }
                last_skipped = false;
            }
            Choice::Undervote => {
                if last_skipped {
                    new_choices.push(Choice::Undervote);
                    break
                }
                last_skipped = true;
            }
            Choice::Overvote => {
                new_choices.push(Choice::Overvote);
                break
            }
        }
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
            maine_normalizer(b)
        );
    }

    #[test]
    fn test_remove_duplicate() {
        let c1 = Choice::Vote(CandidateId(1));
        let c2 = Choice::Vote(CandidateId(2));
        let b = Ballot::new("1".into(), vec![c1, c2, c1]);

        assert_eq!(
            Ballot::new("1".into(), vec![c1, c2]),
            maine_normalizer(b)
        );
    }

    #[test]
    fn test_remove_multiple() {
        let c1 = Choice::Vote(CandidateId(1));
        let b = Ballot::new("1".into(), vec![c1, c1, c1, c1]);

        assert_eq!(
            Ballot::new("1".into(), vec![c1]),
            maine_normalizer(b)
        );
    }

    #[test]
    fn test_undervote() {
        let c1 = Choice::Vote(CandidateId(1));
        let c2 = Choice::Vote(CandidateId(2));
        let b = Ballot::new("1".into(), vec![c1, Choice::Undervote, c2]);

        assert_eq!(
            Ballot::new("1".into(), vec![c1, c2]),
            maine_normalizer(b)
        );
    }

    #[test]
    fn test_overvote() {
        let c1 = Choice::Vote(CandidateId(1));
        let c2 = Choice::Vote(CandidateId(2));
        let b = Ballot::new("1".into(), vec![c1, Choice::Overvote, c2]);

        assert_eq!(
            Ballot::new("1".into(), vec![c1, Choice::Overvote]),
            maine_normalizer(b)
        );
    }

    #[test]
    fn test_skipped_vote() {
        let c1 = Choice::Vote(CandidateId(1));
        let c2 = Choice::Vote(CandidateId(2));
        let b = Ballot::new("1".into(), vec![c1, Choice::Undervote, c2]);

        assert_eq!(
            Ballot::new("1".into(), vec![c1, c2]),
            maine_normalizer(b)
        );
    }

    #[test]
    fn test_two_skipped_vote() {
        let c1 = Choice::Vote(CandidateId(1));
        let c2 = Choice::Vote(CandidateId(2));
        let b = Ballot::new("1".into(), vec![c1, Choice::Undervote, Choice::Undervote, c2]);

        assert_eq!(
            Ballot::new("1".into(), vec![c1, Choice::Undervote]),
            maine_normalizer(b)
        );
    }

    #[test]
    fn test_two_nonsequential_skipped_vote() {
        let c1 = Choice::Vote(CandidateId(1));
        let c2 = Choice::Vote(CandidateId(2));
        let c3 = Choice::Vote(CandidateId(3));
        let b = Ballot::new("1".into(), vec![c1, Choice::Undervote, c2, Choice::Undervote, c3]);

        assert_eq!(
            Ballot::new("1".into(), vec![c1, c2, c3]),
            maine_normalizer(b)
        );
    }
}
