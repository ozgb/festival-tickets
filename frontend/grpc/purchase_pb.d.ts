import * as jspb from 'google-protobuf'



export class Order extends jspb.Message {
  getId(): string;
  setId(value: string): Order;

  getType(): TicketType | undefined;
  setType(value?: TicketType): Order;
  hasType(): boolean;
  clearType(): Order;

  getDuration(): number;
  setDuration(value: number): Order;

  getPrice(): number;
  setPrice(value: number): Order;

  getReservedUntil(): number;
  setReservedUntil(value: number): Order;

  getPurchasedAt(): number;
  setPurchasedAt(value: number): Order;
  hasPurchasedAt(): boolean;
  clearPurchasedAt(): Order;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Order.AsObject;
  static toObject(includeInstance: boolean, msg: Order): Order.AsObject;
  static serializeBinaryToWriter(message: Order, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Order;
  static deserializeBinaryFromReader(message: Order, reader: jspb.BinaryReader): Order;
}

export namespace Order {
  export type AsObject = {
    id: string,
    type?: TicketType.AsObject,
    duration: number,
    price: number,
    reservedUntil: number,
    purchasedAt?: number,
  }

  export enum PurchasedAtCase { 
    _PURCHASED_AT_NOT_SET = 0,
    PURCHASED_AT = 6,
  }
}

export class TicketType extends jspb.Message {
  getId(): string;
  setId(value: string): TicketType;

  getDisplay(): string;
  setDisplay(value: string): TicketType;

  getSoldOut(): boolean;
  setSoldOut(value: boolean): TicketType;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): TicketType.AsObject;
  static toObject(includeInstance: boolean, msg: TicketType): TicketType.AsObject;
  static serializeBinaryToWriter(message: TicketType, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): TicketType;
  static deserializeBinaryFromReader(message: TicketType, reader: jspb.BinaryReader): TicketType;
}

export namespace TicketType {
  export type AsObject = {
    id: string,
    display: string,
    soldOut: boolean,
  }
}

export class OrderStats extends jspb.Message {
  getDurationDays(): number;
  setDurationDays(value: number): OrderStats;

  getOrderLimit(): number;
  setOrderLimit(value: number): OrderStats;

  getOrderCount(): number;
  setOrderCount(value: number): OrderStats;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): OrderStats.AsObject;
  static toObject(includeInstance: boolean, msg: OrderStats): OrderStats.AsObject;
  static serializeBinaryToWriter(message: OrderStats, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): OrderStats;
  static deserializeBinaryFromReader(message: OrderStats, reader: jspb.BinaryReader): OrderStats;
}

export namespace OrderStats {
  export type AsObject = {
    durationDays: number,
    orderLimit: number,
    orderCount: number,
  }
}

export class GetOrderStatsRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetOrderStatsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetOrderStatsRequest): GetOrderStatsRequest.AsObject;
  static serializeBinaryToWriter(message: GetOrderStatsRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetOrderStatsRequest;
  static deserializeBinaryFromReader(message: GetOrderStatsRequest, reader: jspb.BinaryReader): GetOrderStatsRequest;
}

export namespace GetOrderStatsRequest {
  export type AsObject = {
  }
}

export class GetOrderStatsResponse extends jspb.Message {
  getOrderStatsList(): Array<OrderStats>;
  setOrderStatsList(value: Array<OrderStats>): GetOrderStatsResponse;
  clearOrderStatsList(): GetOrderStatsResponse;
  addOrderStats(value?: OrderStats, index?: number): OrderStats;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetOrderStatsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetOrderStatsResponse): GetOrderStatsResponse.AsObject;
  static serializeBinaryToWriter(message: GetOrderStatsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetOrderStatsResponse;
  static deserializeBinaryFromReader(message: GetOrderStatsResponse, reader: jspb.BinaryReader): GetOrderStatsResponse;
}

export namespace GetOrderStatsResponse {
  export type AsObject = {
    orderStatsList: Array<OrderStats.AsObject>,
  }
}

export class GetOrderRequest extends jspb.Message {
  getId(): string;
  setId(value: string): GetOrderRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetOrderRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetOrderRequest): GetOrderRequest.AsObject;
  static serializeBinaryToWriter(message: GetOrderRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetOrderRequest;
  static deserializeBinaryFromReader(message: GetOrderRequest, reader: jspb.BinaryReader): GetOrderRequest;
}

export namespace GetOrderRequest {
  export type AsObject = {
    id: string,
  }
}

export class GetOrderResponse extends jspb.Message {
  getOrder(): Order | undefined;
  setOrder(value?: Order): GetOrderResponse;
  hasOrder(): boolean;
  clearOrder(): GetOrderResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetOrderResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetOrderResponse): GetOrderResponse.AsObject;
  static serializeBinaryToWriter(message: GetOrderResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetOrderResponse;
  static deserializeBinaryFromReader(message: GetOrderResponse, reader: jspb.BinaryReader): GetOrderResponse;
}

export namespace GetOrderResponse {
  export type AsObject = {
    order?: Order.AsObject,
  }
}

export class PurchaseOrderRequest extends jspb.Message {
  getId(): string;
  setId(value: string): PurchaseOrderRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PurchaseOrderRequest.AsObject;
  static toObject(includeInstance: boolean, msg: PurchaseOrderRequest): PurchaseOrderRequest.AsObject;
  static serializeBinaryToWriter(message: PurchaseOrderRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PurchaseOrderRequest;
  static deserializeBinaryFromReader(message: PurchaseOrderRequest, reader: jspb.BinaryReader): PurchaseOrderRequest;
}

export namespace PurchaseOrderRequest {
  export type AsObject = {
    id: string,
  }
}

export class PurchaseOrderResponse extends jspb.Message {
  getOrder(): Order | undefined;
  setOrder(value?: Order): PurchaseOrderResponse;
  hasOrder(): boolean;
  clearOrder(): PurchaseOrderResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PurchaseOrderResponse.AsObject;
  static toObject(includeInstance: boolean, msg: PurchaseOrderResponse): PurchaseOrderResponse.AsObject;
  static serializeBinaryToWriter(message: PurchaseOrderResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PurchaseOrderResponse;
  static deserializeBinaryFromReader(message: PurchaseOrderResponse, reader: jspb.BinaryReader): PurchaseOrderResponse;
}

export namespace PurchaseOrderResponse {
  export type AsObject = {
    order?: Order.AsObject,
  }
}

export class GetTicketTypesRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetTicketTypesRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetTicketTypesRequest): GetTicketTypesRequest.AsObject;
  static serializeBinaryToWriter(message: GetTicketTypesRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetTicketTypesRequest;
  static deserializeBinaryFromReader(message: GetTicketTypesRequest, reader: jspb.BinaryReader): GetTicketTypesRequest;
}

export namespace GetTicketTypesRequest {
  export type AsObject = {
  }
}

export class GetTicketTypesResponse extends jspb.Message {
  getTicketTypesList(): Array<TicketType>;
  setTicketTypesList(value: Array<TicketType>): GetTicketTypesResponse;
  clearTicketTypesList(): GetTicketTypesResponse;
  addTicketTypes(value?: TicketType, index?: number): TicketType;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetTicketTypesResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetTicketTypesResponse): GetTicketTypesResponse.AsObject;
  static serializeBinaryToWriter(message: GetTicketTypesResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetTicketTypesResponse;
  static deserializeBinaryFromReader(message: GetTicketTypesResponse, reader: jspb.BinaryReader): GetTicketTypesResponse;
}

export namespace GetTicketTypesResponse {
  export type AsObject = {
    ticketTypesList: Array<TicketType.AsObject>,
  }
}

export class GetTicketDurationsRequest extends jspb.Message {
  getTicketTypeId(): string;
  setTicketTypeId(value: string): GetTicketDurationsRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetTicketDurationsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetTicketDurationsRequest): GetTicketDurationsRequest.AsObject;
  static serializeBinaryToWriter(message: GetTicketDurationsRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetTicketDurationsRequest;
  static deserializeBinaryFromReader(message: GetTicketDurationsRequest, reader: jspb.BinaryReader): GetTicketDurationsRequest;
}

export namespace GetTicketDurationsRequest {
  export type AsObject = {
    ticketTypeId: string,
  }
}

export class GetTicketDurationsResponse extends jspb.Message {
  getTicketDurationsList(): Array<number>;
  setTicketDurationsList(value: Array<number>): GetTicketDurationsResponse;
  clearTicketDurationsList(): GetTicketDurationsResponse;
  addTicketDurations(value: number, index?: number): GetTicketDurationsResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetTicketDurationsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetTicketDurationsResponse): GetTicketDurationsResponse.AsObject;
  static serializeBinaryToWriter(message: GetTicketDurationsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetTicketDurationsResponse;
  static deserializeBinaryFromReader(message: GetTicketDurationsResponse, reader: jspb.BinaryReader): GetTicketDurationsResponse;
}

export namespace GetTicketDurationsResponse {
  export type AsObject = {
    ticketDurationsList: Array<number>,
  }
}

export class AddTicketToBasketRequest extends jspb.Message {
  getTicketTypeId(): string;
  setTicketTypeId(value: string): AddTicketToBasketRequest;

  getDuration(): number;
  setDuration(value: number): AddTicketToBasketRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AddTicketToBasketRequest.AsObject;
  static toObject(includeInstance: boolean, msg: AddTicketToBasketRequest): AddTicketToBasketRequest.AsObject;
  static serializeBinaryToWriter(message: AddTicketToBasketRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AddTicketToBasketRequest;
  static deserializeBinaryFromReader(message: AddTicketToBasketRequest, reader: jspb.BinaryReader): AddTicketToBasketRequest;
}

export namespace AddTicketToBasketRequest {
  export type AsObject = {
    ticketTypeId: string,
    duration: number,
  }
}

export class AddTicketToBasketResponse extends jspb.Message {
  getOrder(): Order | undefined;
  setOrder(value?: Order): AddTicketToBasketResponse;
  hasOrder(): boolean;
  clearOrder(): AddTicketToBasketResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AddTicketToBasketResponse.AsObject;
  static toObject(includeInstance: boolean, msg: AddTicketToBasketResponse): AddTicketToBasketResponse.AsObject;
  static serializeBinaryToWriter(message: AddTicketToBasketResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AddTicketToBasketResponse;
  static deserializeBinaryFromReader(message: AddTicketToBasketResponse, reader: jspb.BinaryReader): AddTicketToBasketResponse;
}

export namespace AddTicketToBasketResponse {
  export type AsObject = {
    order?: Order.AsObject,
  }
}

