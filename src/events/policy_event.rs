use leap_sys::LEAP_POLICY_EVENT;

use crate::PolicyFlags;

crate::leap_ref_struct!(PolicyEvent, LEAP_POLICY_EVENT);

impl<'a> PolicyEvent<'a> {
    pub fn current_policy(&self) -> Option<PolicyFlags> {
        PolicyFlags::from_bits(self.handle.current_policy.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::*;
    use crate::*;
    #[test]
    fn policy_event() {
        let mut connection = initialize_connection();
        connection
            .set_policy_flags(PolicyFlags::IMAGES, PolicyFlags::empty())
            .expect("Failed to set policy");
        connection.expect_event(
            "Did not receive an image policy event".to_string(),
            |e| match e {
                Event::Policy(e) => {
                    if e.current_policy()
                        .expect("Failed to parse a policy flag")
                        .contains(PolicyFlags::IMAGES)
                    {
                        Some(())
                    } else {
                        None
                    }
                }
                _ => None,
            },
        );
    }
}
