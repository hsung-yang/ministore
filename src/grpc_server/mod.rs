use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tonic::transport::Server;
use tonic::Response;

use crate::device_manager::DeviceManager;

use self::ministore_proto::mini_service_server::{MiniService, MiniServiceServer};
use self::ministore_proto::{
    CreateFakeDeviceRequest, CreateFakeDeviceResponse, DeleteFakeDeviceRequest,
    DeleteFakeDeviceResponse, FakeDevice, ListFakeDevicesRequest, ListFakeDevicesResponse, Status,
    StatusRequest, StatusResponse,
};

pub mod ministore_proto {
    tonic::include_proto!("ministore");
}

pub async fn start_grpc_server(addr: SocketAddr) -> Result<(), String> {
    todo!()
}

#[derive(Default)]
pub struct GrpcServer {}

#[tonic::async_trait]
impl MiniService for GrpcServer {
    async fn status(
        &self,
        request: tonic::Request<StatusRequest>,
    ) -> Result<tonic::Response<StatusResponse>, tonic::Status> {
        todo!()
    }

    async fn create_fake_device(
        &self,
        request: tonic::Request<CreateFakeDeviceRequest>,
    ) -> Result<tonic::Response<CreateFakeDeviceResponse>, tonic::Status> {
        todo!()
    }

    async fn delete_fake_device(
        &self,
        request: tonic::Request<DeleteFakeDeviceRequest>,
    ) -> Result<tonic::Response<DeleteFakeDeviceResponse>, tonic::Status> {
        todo!()
    }

    async fn list_fake_devices(
        &self,
        request: tonic::Request<ListFakeDevicesRequest>,
    ) -> Result<tonic::Response<ListFakeDevicesResponse>, tonic::Status> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        grpc_server::ministore_proto::mini_service_client::MiniServiceClient,
        utils::humansize_to_integer,
    };

    use super::*;

    #[tokio::test]
    async fn server_should_response_with_ready_when_started() {
        let addr = "127.0.0.1:8080";
        let addr_for_server = addr.parse().expect("Failed to parse socket addr");
        let addr_for_client = format!("http://{}", addr.clone());

        let start_server = tokio::spawn(async move {
            start_grpc_server(addr_for_server)
                .await
                .expect("Failed to start grpc server");
        });

        let test = tokio::spawn(async move {
            let mut client = MiniServiceClient::connect(addr_for_client)
                .await
                .expect("Failed to start test client");
            let request: tonic::Request<StatusRequest> = tonic::Request::new(StatusRequest {});

            let response = client
                .status(request)
                .await
                .expect("Failed to get response");
            assert_eq!(response.into_inner().status, Status::Ready as i32);
        });

        assert_eq!(start_server.is_finished(), true);
        assert_eq!(test.is_finished(), true);
    }

    #[tokio::test]
    async fn server_can_create_and_delete_fake_device() {
        let addr = "127.0.0.1:8081";
        let addr_for_server = addr.parse().expect("Failed to parse socket addr");
        let addr_for_client = format!("http://{}", addr.clone());

        let start_server = tokio::spawn(async move {
            start_grpc_server(addr_for_server)
                .await
                .expect("Failed to start grpc server");
        });

        let test = tokio::spawn(async move {
            let mut client = MiniServiceClient::connect(addr_for_client)
                .await
                .expect("Failed to start test client");

            // Create device and verify it using list devices
            let device_name = "server_can_create_and_delete_fake_device".to_string();
            let request = tonic::Request::new(CreateFakeDeviceRequest {
                name: device_name.clone(),
                size: humansize_to_integer("1M").unwrap(),
                device_type: 0, // SimpleFakeDevice
            });
            let response = client
                .create_fake_device(request)
                .await
                .expect("Failed to create fake device");
            let response = response.into_inner();
            assert_eq!(response.success, true);

            let request = tonic::Request::new(ListFakeDevicesRequest {});
            let response = client
                .list_fake_devices(request)
                .await
                .expect("Failed to get response");
            let response = response.into_inner();

            assert_eq!(response.success, true);
            assert_eq!(response.device_list.len(), 1);
            assert_eq!(response.device_list.get(0).unwrap().name, device_name);
            assert_eq!(
                response.device_list.get(0).unwrap().size,
                humansize_to_integer("1M").unwrap()
            );
            assert_eq!(response.device_list.get(0).unwrap().device_type, 0);

            // Delete device and verify it using list devices
            let request = tonic::Request::new(DeleteFakeDeviceRequest {
                name: device_name.clone(),
            });
            let response = client
                .delete_fake_device(request)
                .await
                .expect("Failed to delete fake device");
            let response = response.into_inner();
            assert_eq!(response.success, true);

            let request = tonic::Request::new(ListFakeDevicesRequest {});
            let response = client
                .list_fake_devices(request)
                .await
                .expect("Failed to get response");
            let response = response.into_inner();

            assert_eq!(response.success, true);
            assert_eq!(response.device_list.len(), 0);
        });

        assert_eq!(start_server.is_finished(), true);
        assert_eq!(test.is_finished(), true);
    }
}
