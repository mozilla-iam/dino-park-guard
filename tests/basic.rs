#[macro_use]
extern crate dino_park_guard;
use dino_park_trust::AALevel;
use dino_park_trust::AALevelError;
use dino_park_trust::GroupsTrust;
use dino_park_trust::GroupsTrustError;
use dino_park_trust::Trust;
use dino_park_trust::TrustError;
use failure::Error;

#[guard(Staff)]
fn something(b: String) -> Result<String, Error> {
    Ok(b)
}

#[guard(Staff, Creator)]
fn something_groups() -> Result<(), Error> {
    Ok(())
}

#[guard(Staff, None, High)]
fn something_aal() -> Result<(), Error> {
    Ok(())
}

#[test]
fn fail_me() -> Result<(), Error> {
    use dino_park_gate::scope::ScopeAndUser;
    let scope_and_user = ScopeAndUser {
        user_id: "foobar".to_owned(),
        scope: Trust::Ndaed,
        groups_scope: GroupsTrust::None,
        aa_level: AALevel::Unknown,
    };
    let ret = something("b".to_owned(), scope_and_user);
    assert_eq!(
        ret.err().unwrap().downcast::<TrustError>().unwrap(),
        TrustError::TrustLevelToLow
    );
    Ok(())
}

#[test]
fn test_me() -> Result<(), Error> {
    use dino_park_gate::scope::ScopeAndUser;
    let scope_and_user = ScopeAndUser {
        user_id: "foobar".to_owned(),
        scope: Trust::Staff,
        groups_scope: GroupsTrust::None,
        aa_level: AALevel::Unknown,
    };
    something("b".to_owned(), scope_and_user)?;
    Ok(())
}

#[test]
fn fail_me_group() -> Result<(), Error> {
    use dino_park_gate::scope::ScopeAndUser;
    let scope_and_user = ScopeAndUser {
        user_id: "foobar".to_owned(),
        scope: Trust::Staff,
        groups_scope: GroupsTrust::None,
        aa_level: AALevel::Unknown,
    };
    let ret = something_groups(scope_and_user);
    assert_eq!(
        ret.err().unwrap().downcast::<GroupsTrustError>().unwrap(),
        GroupsTrustError::GroupsTrustLevelToLow
    );
    Ok(())
}

#[test]
fn test_me_aal() -> Result<(), Error> {
    use dino_park_gate::scope::ScopeAndUser;
    let scope_and_user = ScopeAndUser {
        user_id: "foobar".to_owned(),
        scope: Trust::Staff,
        groups_scope: GroupsTrust::None,
        aa_level: AALevel::Maximum,
    };
    something_aal(scope_and_user)?;
    Ok(())
}

#[test]
fn fail_me_aal() -> Result<(), Error> {
    use dino_park_gate::scope::ScopeAndUser;
    let scope_and_user = ScopeAndUser {
        user_id: "foobar".to_owned(),
        scope: Trust::Staff,
        groups_scope: GroupsTrust::Admin,
        aa_level: AALevel::Unknown,
    };
    let ret = something_aal(scope_and_user);
    assert_eq!(
        ret.err().unwrap().downcast::<AALevelError>().unwrap(),
        AALevelError::AALevelToLow
    );
    Ok(())
}
