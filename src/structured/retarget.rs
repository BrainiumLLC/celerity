// This macro generates the code required for smoothly interrupting
// an animation with a new destination.
#[macro_export]
macro_rules! retarget_function {
    ( $anim:ident, $v:ty ) => {
        pub fn $anim(
            &mut self,
            interrupt_t: Duration,
            transition_t: Duration,
            target: $v,
            ease: Option<BezierEase>,
        ) {
            let interrupt_v = self.$anim.sample(interrupt_t);

            replace_with::replace_with_or_abort(&mut self.$anim, |anim| {
                Box::new(anim.interrupt(
                    Interval::from_values(transition_t, interrupt_v, target, ease),
                    interrupt_t,
                    transition_t,
                ))
            });
        }

        paste::paste! {
            pub fn [<$anim _animation>](
                &mut self,
                interrupt_t: Duration,
                transition_t: Duration,
                new_animation: Box<dyn Animation<$v>>,
            ) {
                self.$anim.replace_with(|anim| {
                    Box::new(anim.interrupt(new_animation, interrupt_t, transition_t))
                });
            }
        }
    };
}
