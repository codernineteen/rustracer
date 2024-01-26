#[macro_export]
macro_rules! check_func_perf {
    ($func_name:expr) => {
        let now = std::time::Instant::now();
        $func_name;
        let elapsed_time = now.elapsed();

        let function_call = stringify!($func_name);
        let tokens = function_call.split(".");
        let token_collection = tokens.collect::<Vec<&str>>();

        if elapsed_time.as_secs() <= 0 {
            match token_collection.len() > 1 {
                true => println!(
                    "[performance notice] : {} took {}ms.\n",
                    token_collection[1],
                    elapsed_time.as_millis()
                ),
                false => println!(
                    "[performance notice] : {} took {}ms.\n",
                    stringify!($func_name),
                    elapsed_time.as_millis()
                ),
            }
        } else {
            match token_collection.len() > 1 {
                true => println!(
                    "[performance notice] : {} took {}ms.\n",
                    token_collection[1],
                    elapsed_time.as_secs()
                ),
                false => println!(
                    "[performance notice] : {} took {}ms.\n",
                    stringify!($func_name),
                    elapsed_time.as_secs()
                ),
            }
        }
    };
}
