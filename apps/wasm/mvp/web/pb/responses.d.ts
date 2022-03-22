import * as $protobuf from "protobufjs";
/** Properties of a CommonResponse. */
export interface ICommonResponse {

    /** CommonResponse success */
    success?: (boolean|null);

    /** CommonResponse errorMessage */
    errorMessage?: (string|null);
}

/** Represents a CommonResponse. */
export class CommonResponse implements ICommonResponse {

    /**
     * Constructs a new CommonResponse.
     * @param [properties] Properties to set
     */
    constructor(properties?: ICommonResponse);

    /** CommonResponse success. */
    public success: boolean;

    /** CommonResponse errorMessage. */
    public errorMessage: string;

    /**
     * Creates a new CommonResponse instance using the specified properties.
     * @param [properties] Properties to set
     * @returns CommonResponse instance
     */
    public static create(properties?: ICommonResponse): CommonResponse;

    /**
     * Encodes the specified CommonResponse message. Does not implicitly {@link CommonResponse.verify|verify} messages.
     * @param message CommonResponse message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: ICommonResponse, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified CommonResponse message, length delimited. Does not implicitly {@link CommonResponse.verify|verify} messages.
     * @param message CommonResponse message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: ICommonResponse, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a CommonResponse message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns CommonResponse
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): CommonResponse;

    /**
     * Decodes a CommonResponse message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns CommonResponse
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): CommonResponse;

    /**
     * Verifies a CommonResponse message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a CommonResponse message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns CommonResponse
     */
    public static fromObject(object: { [k: string]: any }): CommonResponse;

    /**
     * Creates a plain object from a CommonResponse message. Also converts values to other types if specified.
     * @param message CommonResponse
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: CommonResponse, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this CommonResponse to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}
