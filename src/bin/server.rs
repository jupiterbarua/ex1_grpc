extern crate ex1_grpc;
extern crate grpc;
extern crate protobuf;

use std::thread;

use ex1_grpc::myex_grpc::*;
use ex1_grpc::myex::*;

struct MyExServer;

impl MyExService for MyExServer {
    fn record_cab_location(&self, _m: grpc::RequestOptins, req: CabLocationRequest)-> 
        grpc::SingleResponse<CabLocationResponse> {
            let mut r = CabLocationResponse::new();
            println!("Record cab {} at {}, {}", req.get_name(), req.get_location().latitude,
                    req.get_location().longtitude);
            r.set_accepted(true);
            grpc::SingleResponse::completed(r)
        }

    fn get_cabs(&self, _m: grpc::RequestOptions,
                        _req: GetCabRequest)-> 
                        grpc::SingleResponse<GetCabResponse>{
            let mut r = GetCabResponse::new();
            
            let mut location = Location::new();
            location.latitude = 40.7128;
            location.longtitude = -74.0060;
            
            let mut one = Cab::new();
            one.set_name("Merec".to_owned());
            one.set_location(location.clone());

            let mut two = Cab::new();
            two.set_name("Limo".to_owned());
            two.set_location(location.clone());
            let vec = vec![one,two];
            let cabs = ::protobuf::RepeatedField::from_vec(vec);
            r.set_cabs(cabs);
            grpc::SingleResponse::completed(r)

            }    
}

fn main(){
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(9001);
    server.add_service(FooBarServiceServer::new_service_def(FooBarServer));
    server.http.set_cpu_pool_threads(4);
    let _server = server.build().expect("could not start server");
    loop {
        thread::park();
    }
}