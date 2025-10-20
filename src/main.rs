mod metric_clock;

fn main() {
    let (h,m,s)= metric_clock::get_metric_time(2);
    println!("Metric Time: {}:{}:{}", h, m, s);
}
