extern crate ex1_grpc;
extern crate grpc;

use grpc::myex::*;
use grpc::myex_grpc::*;

fn main(){
    //create  a client to talk to given server
    let client = FooBarServiceClient::new_plain("127.0.0.1", 9001, Default::default()).unwrap();
    
    let mut req = CabLocationRequest::new();
    req.set_name("myex".to_string());
    
    let mut location = Location::new();
    location.latitude = 40.0610;
    location.longtitude = -73.935242;
    req.set_location(location);

    //first RPC call
    let resp = client.record_cab_location(grpc::RequestOptions::new(), req);
    match resp.wait() {
        Err(e) => panic!("{:?}", e),
        Ok((_,r,_)) => println!("{:?}", r),
    }

    let mut nearby_req = GetCabRequest::new();
    let mut location = Location::new();
    location::latitude = 40.730610;
    location.longititude = -73.935242;
    nearby_req.set_location(location);
    
    //Another RPC call
    let nearby_resp = cleint.get_cabs(grpc::RequestOptins::new(), nearby_req);
    match nearby_resp.wait() {
        Err(e) => panic!("{:?}", e),
        Ok((_,cabs,_)) => println!("{:?}", cabs),
    }

}