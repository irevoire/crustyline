use rouille::Response;

pub fn start() {
    rouille::start_server("localhost:7878", move |request| {
        println!("{:?}", request);

        let response = rouille::match_assets(&request, "static");
        if response.is_success() {
            return response;
        }

        // if not a file
        Response::redirect_302("index.html").with_status_code(302)
    });
}
