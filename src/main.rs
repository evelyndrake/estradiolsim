use termplot::*;
fn main() {
    let mut plot = Plot::default();
    plot.set_domain(Domain(-10.0..10.0))
        .set_codomain(Domain(-0.3..1.2))
        .set_title("Graph title")
        .set_x_label("X axis")
        .set_y_label("Y axis")
        .set_size(Size::new(175, 50))
        .add_plot(Box::new(plot::Graph::new(|x| x.sin() / x)));
    
    println!("{plot}");    
}
