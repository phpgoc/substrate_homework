parameter_types! {
	pub const StringLimit: u32 = 50;
}
impl pallet_poe::Config for Test {
	type Event = Event;
	type StringLimit =  StringLimit;
}

