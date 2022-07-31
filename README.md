# weekend
![Image](/images/weekend.jpg)

A Rust implementation of Peter Shirley's [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

## Building
I recommend building in release mode, as it speeds the rendering up significantly.
```
cargo build --release
```
Upon running the program, a progress bar powered by [`indicatif`](https://github.com/console-rs/indicatif) will show the rendering progress per scanline.
## Performance
The image above rendered in ~22 minutes on a laptop equipped with an AMD Ryzen 5 4500U. It was rendered at a resolution of 1200x800 with 1000 samples per pixel and a maximum of 50 bounces

In order to improve performance, I used vector classes from [`nalgebra`](https://github.com/dimforge/nalgebra) and scanlines are rendered in parallel using [`rayon`](https://github.com/rayon-rs/rayon).

I plan on working through the other books in [the series](https://raytracing.github.io/) as well, where I hope to get another significant speedup from implementing some spatial acceleration structure.