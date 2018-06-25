use dispatch::sysinfo::*;
use shred::*;
use specs::*;

use metrics::MetricsHandler;
use std::time::Instant;

pub struct SystemWrapper<T>(pub T);

pub struct SystemWrapperData<'a, T>
where
	T: System<'a>,
	T::SystemData: DynamicSystemData<'a>,
{
	pub metrics: ReadExpect<'a, MetricsHandler>,
	pub inner: T::SystemData,
}

impl<'a, T> DynamicSystemData<'a> for SystemWrapperData<'a, T>
where
	T: System<'a>,
	T::SystemData: DynamicSystemData<'a>,
{
	type Accessor = <<T as System<'a>>::SystemData as DynamicSystemData<'a>>::Accessor;

	fn setup(acc: &Self::Accessor, res: &mut Resources) {
		<ReadExpect<'a, MetricsHandler> as SystemData<'a>>::setup(res);
		T::SystemData::setup(acc, res);
	}

	fn fetch(acc: &Self::Accessor, res: &'a Resources) -> Self {
		Self {
			metrics: <ReadExpect<'a, MetricsHandler> as SystemData<'a>>::fetch(res),
			inner: T::SystemData::fetch(acc, res),
		}
	}
}

impl<'a, T> System<'a> for SystemWrapper<T>
where
	T: System<'a> + SystemInfo + Send,
	T::SystemData: DynamicSystemData<'a>,
{
	type SystemData = SystemWrapperData<'a, T>;

	fn setup(&mut self, res: &mut Resources) {
		self.0.setup(res);
	}

	fn run(&mut self, data: Self::SystemData) {
		let SystemWrapperData { metrics, inner } = data;

		let start = Instant::now();

		self.0.run(inner);

		let time = Instant::now() - start;

		metrics.time_duration(T::name(), time).unwrap();

		trace!(
			"System '{}' took {}.{:3} ms",
			T::name(),
			time.as_secs() * 1000 + time.subsec_millis() as u64,
			time.subsec_nanos() % 1000
		);
	}
}
