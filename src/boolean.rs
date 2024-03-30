use crate::clipper2::*;

#[derive(Debug, Copy, Clone)]
pub enum FillRule {
    EvenOdd,
    NonZero,
    Positive,
    Negative,
}

impl From<FillRule> for FillRuleC {
    fn from(fill_rule: FillRule) -> Self {
        match fill_rule {
            FillRule::EvenOdd => FillRuleC_EvenOdd,
            FillRule::NonZero => FillRuleC_NonZero,
            FillRule::Positive => FillRuleC_Positive,
            FillRule::Negative => FillRuleC_Negative,
        }
    }
}

pub fn union(subjects: &Paths, fill_rule: FillRule) -> Paths {
    let fill_rule_c = fill_rule.into();
    let subjects_c = subjects as *const Paths as *mut PathsC;
    let result_paths_c = unsafe { union_c(subjects_c, fill_rule_c) };

    Paths::from_raw_parts(result_paths_c)
}
