import * as $protobuf from "protobufjs";
/** Properties of a ConnectRequest. */
export interface IConnectRequest {

    /** ConnectRequest address */
    address?: (string|null);
}

/** Represents a ConnectRequest. */
export class ConnectRequest implements IConnectRequest {

    /**
     * Constructs a new ConnectRequest.
     * @param [properties] Properties to set
     */
    constructor(properties?: IConnectRequest);

    /** ConnectRequest address. */
    public address: string;

    /**
     * Creates a new ConnectRequest instance using the specified properties.
     * @param [properties] Properties to set
     * @returns ConnectRequest instance
     */
    public static create(properties?: IConnectRequest): ConnectRequest;

    /**
     * Encodes the specified ConnectRequest message. Does not implicitly {@link ConnectRequest.verify|verify} messages.
     * @param message ConnectRequest message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: IConnectRequest, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified ConnectRequest message, length delimited. Does not implicitly {@link ConnectRequest.verify|verify} messages.
     * @param message ConnectRequest message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: IConnectRequest, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a ConnectRequest message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns ConnectRequest
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): ConnectRequest;

    /**
     * Decodes a ConnectRequest message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns ConnectRequest
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): ConnectRequest;

    /**
     * Verifies a ConnectRequest message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a ConnectRequest message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns ConnectRequest
     */
    public static fromObject(object: { [k: string]: any }): ConnectRequest;

    /**
     * Creates a plain object from a ConnectRequest message. Also converts values to other types if specified.
     * @param message ConnectRequest
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: ConnectRequest, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this ConnectRequest to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}
