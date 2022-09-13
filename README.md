# RLottieTest

I made gtk::MediaFile that displays lottie animations using rlottie
but I need help with optimizations, because it uses about 8% of my cpu

[animations.webm](https://user-images.githubusercontent.com/63008755/189936438-2c8e298c-5b4a-4667-b29c-b85e789a21b0.webm)

[path to LottieAnimation](https://github.com/YuraIz/RLottieTest/blob/main/src/lottie_animation.rs)

Example of use

```Rust
let filename = "your_file.tgs"; // You can use json or tgs (gzipped json) formats
let lottie_animation = LottieAnimation::from_filename(&filename); 

lottie_animation.play();

obj.imp().picture.set_paintable(Some(&lottie_animation));
```

you can drag'n drop animations from [data/animations](https://github.com/YuraIz/RLottieTest/tree/main/data/animations) folder
