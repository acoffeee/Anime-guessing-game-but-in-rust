
pub mod rate_limiter {
    pub struct Limiter {
        locked: bool,
        time_remaining: i32
    }
    impl Limiter {
        pub fn new() -> Limiter {
            Limiter{
                locked: false,
                time_remaining: 0
                       }
        }
        fn req(&self)  -> bool{
            if self.locked {
                 return false
            }
            return true
        }
        async fn lock(&mut self) {
            *&mut self.locked = true;
            *&mut self.time_remaining = 60;
            *&mut self.timer_for_rate_limiter().await;
        }
        async fn timer_for_rate_limiter(&mut self) {
            for _ in 1..60 {
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                *&mut self.time_remaining -= 1;
            }
        }
    }
}