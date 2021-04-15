# celerity

Buttery smooth animation toolkit.


## Overview

Celerity implements primitives for precise animation of arbitrary value types.

Celerity is largely compatible with the animation model of Adobe After Effects, relying on Cubic Beziers for both temporal and spatial easing. After Effects animations exported using the [`bodymovin`](https://exchange.adobe.com/creativecloud.details.12557.bodymovin.html) plugin can be imported into celerity. This is very similar to the [Lottie](https://airbnb.design/lottie/) web animation framework.

## Example

```
TODO
```

## Traits

Celerity centers on a few traits:

- `trait Animatable<C>` - A value type that can be used for animation keyframes. Must be able to `lerp(...)` (linear interpolation) and measure shortest `distance_to()` between two values A and B. `C` is the type of the scalar components (e.g. `f32`).
- `trait Animation<V, C>` - A time-changing value `V` that you can `sample(...)` at any point in time.
- `trait BoundedAnimation<V, C>` - An animation with a known duration

## Combinators

Celerity has a set of animation combinators which can be used to produce higher-order animations:

- `Chain<A, B, V, C>` - Play animation A, then play animation B
- `Cutoff<A, V, C>` - Play only part of animation A
- `Cycle<A, V, C>` - Repeat animation A indefinitely
- `Interrupt<B, V, C>` - Interrupt an animation A in the middle and transition into a smooth animation B
- `Rev<A, V, C>` - Reverse a bounded animation

## Keyframes vs Intervals

In the API, there are two ways to specify track animations:
1) A user-friendly `Keyframe` API
2) A code-friendly `Interval` API.

In the first, an animation `Track` contains `Keyframe`s with values at specific points in time. This representation is easiest to define and edit, with a single source of truth for each value.

In the second, an animation contains `Interval`s, each of which is a self-contained data structure. This representation is optimized for playback. It describes the entire animation between time `t1` and `t2`, with no dependency on the interval before or after.

