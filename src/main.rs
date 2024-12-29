use quake_clone::run;

fn main() {
    pollster::block_on(run());
}
