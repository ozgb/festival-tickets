const {GetOrderResponse, GetOrderRequest} = require('./purchase_pb.js');
const {ProductServiceClient} = require('./purchase_grpc_web_pb.js');

var client = new ProductServiceClient('http://localhost:8080');

var request = new GetOrderRequest();
request.setId('2a64a953-515e-4b77-90e2-4250be54df67');

client.getOrder(request, {}, (err, response) => {
  console.log(response);
});
