import {GetOrderResponse, GetOrderRequest} from '../grpc/purchase_pb.js';
import {ProductServiceClient} from '../grpc/PurchaseServiceClientPb';
import * as grpcWeb from "grpc-web";
import {createApp, ref} from "vue";

var client = new ProductServiceClient('http://localhost:8080', {}, {});

var request = new GetOrderRequest();
request.setId('2a64a953-515e-4b77-90e2-4250be54df67');


createApp({
  setup() {
    const message = ref('Hello vue!')

    client.getOrder(request, {}, (err: grpcWeb.RpcError, response: GetOrderResponse) => {
      let order = response.getOrder();
      message.value = order.getId();
      //console.log(order.getId());
    });

    return {
      message
    }
  }
}).mount('#app')
