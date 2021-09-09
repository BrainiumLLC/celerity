// This macro generates the code required for smoothly interrupting
// an animation with a new destination or animation.
//
// Simply add to any struct with members that implement Animation:
// retargetable!([member_identifier], [animation_type], [animatable_type]);

#[macro_export]
macro_rules! retargetable {
    ( $anim:ident, $a:ty, $v:ty ) => {
        pub fn $anim(
            &mut self,
            interrupt_t: Duration,
            transition_t: Duration,
            target: $v,
            ease: Option<BezierEase>,
        ) {
            let interrupt_v = self.$anim.sample(interrupt_t);

            self.$anim.replace_with(|anim| {
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
                new_animation: Box<dyn $a<$v>>,
            ) {
                self.$anim.replace_with(|anim| {
                    Box::new(anim.interrupt(new_animation, interrupt_t, transition_t))
                });
            }
        }
    };
}
