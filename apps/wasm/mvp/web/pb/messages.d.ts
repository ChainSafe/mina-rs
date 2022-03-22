import * as $protobuf from "protobufjs";
/** Properties of a PeerStatus. */
export interface IPeerStatus {

    /** PeerStatus connected */
    connected?: (boolean|null);

    /** PeerStatus peerId */
    peerId?: (string|null);

    /** PeerStatus syncStatus */
    syncStatus?: (string|null);

    /** PeerStatus protocolStateHash */
    protocolStateHash?: (string|null);

    /** PeerStatus gitCommit */
    gitCommit?: (string|null);

    /** PeerStatus uptimeMinutes */
    uptimeMinutes?: (number|Long|null);
}

/** Represents a PeerStatus. */
export class PeerStatus implements IPeerStatus {

    /**
     * Constructs a new PeerStatus.
     * @param [properties] Properties to set
     */
    constructor(properties?: IPeerStatus);

    /** PeerStatus connected. */
    public connected: boolean;

    /** PeerStatus peerId. */
    public peerId: string;

    /** PeerStatus syncStatus. */
    public syncStatus: string;

    /** PeerStatus protocolStateHash. */
    public protocolStateHash: string;

    /** PeerStatus gitCommit. */
    public gitCommit: string;

    /** PeerStatus uptimeMinutes. */
    public uptimeMinutes: (number|Long);

    /**
     * Creates a new PeerStatus instance using the specified properties.
     * @param [properties] Properties to set
     * @returns PeerStatus instance
     */
    public static create(properties?: IPeerStatus): PeerStatus;

    /**
     * Encodes the specified PeerStatus message. Does not implicitly {@link PeerStatus.verify|verify} messages.
     * @param message PeerStatus message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: IPeerStatus, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified PeerStatus message, length delimited. Does not implicitly {@link PeerStatus.verify|verify} messages.
     * @param message PeerStatus message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: IPeerStatus, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a PeerStatus message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns PeerStatus
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): PeerStatus;

    /**
     * Decodes a PeerStatus message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns PeerStatus
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): PeerStatus;

    /**
     * Verifies a PeerStatus message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a PeerStatus message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns PeerStatus
     */
    public static fromObject(object: { [k: string]: any }): PeerStatus;

    /**
     * Creates a plain object from a PeerStatus message. Also converts values to other types if specified.
     * @param message PeerStatus
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: PeerStatus, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this PeerStatus to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}
