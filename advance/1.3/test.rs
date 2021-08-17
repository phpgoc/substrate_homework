#[test]
fn create_claim_claim_too_long(){
    new_test_ext().execute_with(|| {
        let claim = vec![1;51];
        assert_noop!(PoeModule::create_claim(Origin::signed(1), claim),Error::<Test>::BadMetadata);
    })
}

