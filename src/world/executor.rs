use std::sync::Arc;
use rayon::ThreadPool;

pub struct Executor {
	thread_pool: Arc<ThreadPool>
}