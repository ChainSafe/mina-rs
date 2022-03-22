/*eslint-disable*/
(function(global, factory) { /* global define, require, module */

    /* AMD */ if (typeof define === 'function' && define.amd)
        define(["protobufjs/minimal"], factory);

    /* CommonJS */ else if (typeof require === 'function' && typeof module === 'object' && module && module.exports)
        module.exports = factory(require("protobufjs/minimal"));

})(this, function($protobuf) {
    "use strict";

    // Common aliases
    var $Reader = $protobuf.Reader, $Writer = $protobuf.Writer, $util = $protobuf.util;
    
    // Exported root namespace
    var $root = $protobuf.roots["default"] || ($protobuf.roots["default"] = {});
    
    $root.PeerStatus = (function() {
    
        /**
         * Properties of a PeerStatus.
         * @exports IPeerStatus
         * @interface IPeerStatus
         * @property {boolean|null} [connected] PeerStatus connected
         * @property {string|null} [peerId] PeerStatus peerId
         * @property {string|null} [syncStatus] PeerStatus syncStatus
         * @property {string|null} [protocolStateHash] PeerStatus protocolStateHash
         * @property {string|null} [gitCommit] PeerStatus gitCommit
         * @property {number|Long|null} [uptimeMinutes] PeerStatus uptimeMinutes
         */
    
        /**
         * Constructs a new PeerStatus.
         * @exports PeerStatus
         * @classdesc Represents a PeerStatus.
         * @implements IPeerStatus
         * @constructor
         * @param {IPeerStatus=} [properties] Properties to set
         */
        function PeerStatus(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }
    
        /**
         * PeerStatus connected.
         * @member {boolean} connected
         * @memberof PeerStatus
         * @instance
         */
        PeerStatus.prototype.connected = false;
    
        /**
         * PeerStatus peerId.
         * @member {string} peerId
         * @memberof PeerStatus
         * @instance
         */
        PeerStatus.prototype.peerId = "";
    
        /**
         * PeerStatus syncStatus.
         * @member {string} syncStatus
         * @memberof PeerStatus
         * @instance
         */
        PeerStatus.prototype.syncStatus = "";
    
        /**
         * PeerStatus protocolStateHash.
         * @member {string} protocolStateHash
         * @memberof PeerStatus
         * @instance
         */
        PeerStatus.prototype.protocolStateHash = "";
    
        /**
         * PeerStatus gitCommit.
         * @member {string} gitCommit
         * @memberof PeerStatus
         * @instance
         */
        PeerStatus.prototype.gitCommit = "";
    
        /**
         * PeerStatus uptimeMinutes.
         * @member {number|Long} uptimeMinutes
         * @memberof PeerStatus
         * @instance
         */
        PeerStatus.prototype.uptimeMinutes = $util.Long ? $util.Long.fromBits(0,0,false) : 0;
    
        /**
         * Creates a new PeerStatus instance using the specified properties.
         * @function create
         * @memberof PeerStatus
         * @static
         * @param {IPeerStatus=} [properties] Properties to set
         * @returns {PeerStatus} PeerStatus instance
         */
        PeerStatus.create = function create(properties) {
            return new PeerStatus(properties);
        };
    
        /**
         * Encodes the specified PeerStatus message. Does not implicitly {@link PeerStatus.verify|verify} messages.
         * @function encode
         * @memberof PeerStatus
         * @static
         * @param {IPeerStatus} message PeerStatus message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        PeerStatus.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.connected != null && Object.hasOwnProperty.call(message, "connected"))
                writer.uint32(/* id 1, wireType 0 =*/8).bool(message.connected);
            if (message.peerId != null && Object.hasOwnProperty.call(message, "peerId"))
                writer.uint32(/* id 2, wireType 2 =*/18).string(message.peerId);
            if (message.syncStatus != null && Object.hasOwnProperty.call(message, "syncStatus"))
                writer.uint32(/* id 3, wireType 2 =*/26).string(message.syncStatus);
            if (message.protocolStateHash != null && Object.hasOwnProperty.call(message, "protocolStateHash"))
                writer.uint32(/* id 4, wireType 2 =*/34).string(message.protocolStateHash);
            if (message.gitCommit != null && Object.hasOwnProperty.call(message, "gitCommit"))
                writer.uint32(/* id 5, wireType 2 =*/42).string(message.gitCommit);
            if (message.uptimeMinutes != null && Object.hasOwnProperty.call(message, "uptimeMinutes"))
                writer.uint32(/* id 6, wireType 0 =*/48).int64(message.uptimeMinutes);
            return writer;
        };
    
        /**
         * Encodes the specified PeerStatus message, length delimited. Does not implicitly {@link PeerStatus.verify|verify} messages.
         * @function encodeDelimited
         * @memberof PeerStatus
         * @static
         * @param {IPeerStatus} message PeerStatus message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        PeerStatus.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };
    
        /**
         * Decodes a PeerStatus message from the specified reader or buffer.
         * @function decode
         * @memberof PeerStatus
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {PeerStatus} PeerStatus
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        PeerStatus.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.PeerStatus();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.connected = reader.bool();
                    break;
                case 2:
                    message.peerId = reader.string();
                    break;
                case 3:
                    message.syncStatus = reader.string();
                    break;
                case 4:
                    message.protocolStateHash = reader.string();
                    break;
                case 5:
                    message.gitCommit = reader.string();
                    break;
                case 6:
                    message.uptimeMinutes = reader.int64();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };
    
        /**
         * Decodes a PeerStatus message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof PeerStatus
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {PeerStatus} PeerStatus
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        PeerStatus.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };
    
        /**
         * Verifies a PeerStatus message.
         * @function verify
         * @memberof PeerStatus
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        PeerStatus.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.connected != null && message.hasOwnProperty("connected"))
                if (typeof message.connected !== "boolean")
                    return "connected: boolean expected";
            if (message.peerId != null && message.hasOwnProperty("peerId"))
                if (!$util.isString(message.peerId))
                    return "peerId: string expected";
            if (message.syncStatus != null && message.hasOwnProperty("syncStatus"))
                if (!$util.isString(message.syncStatus))
                    return "syncStatus: string expected";
            if (message.protocolStateHash != null && message.hasOwnProperty("protocolStateHash"))
                if (!$util.isString(message.protocolStateHash))
                    return "protocolStateHash: string expected";
            if (message.gitCommit != null && message.hasOwnProperty("gitCommit"))
                if (!$util.isString(message.gitCommit))
                    return "gitCommit: string expected";
            if (message.uptimeMinutes != null && message.hasOwnProperty("uptimeMinutes"))
                if (!$util.isInteger(message.uptimeMinutes) && !(message.uptimeMinutes && $util.isInteger(message.uptimeMinutes.low) && $util.isInteger(message.uptimeMinutes.high)))
                    return "uptimeMinutes: integer|Long expected";
            return null;
        };
    
        /**
         * Creates a PeerStatus message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof PeerStatus
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {PeerStatus} PeerStatus
         */
        PeerStatus.fromObject = function fromObject(object) {
            if (object instanceof $root.PeerStatus)
                return object;
            var message = new $root.PeerStatus();
            if (object.connected != null)
                message.connected = Boolean(object.connected);
            if (object.peerId != null)
                message.peerId = String(object.peerId);
            if (object.syncStatus != null)
                message.syncStatus = String(object.syncStatus);
            if (object.protocolStateHash != null)
                message.protocolStateHash = String(object.protocolStateHash);
            if (object.gitCommit != null)
                message.gitCommit = String(object.gitCommit);
            if (object.uptimeMinutes != null)
                if ($util.Long)
                    (message.uptimeMinutes = $util.Long.fromValue(object.uptimeMinutes)).unsigned = false;
                else if (typeof object.uptimeMinutes === "string")
                    message.uptimeMinutes = parseInt(object.uptimeMinutes, 10);
                else if (typeof object.uptimeMinutes === "number")
                    message.uptimeMinutes = object.uptimeMinutes;
                else if (typeof object.uptimeMinutes === "object")
                    message.uptimeMinutes = new $util.LongBits(object.uptimeMinutes.low >>> 0, object.uptimeMinutes.high >>> 0).toNumber();
            return message;
        };
    
        /**
         * Creates a plain object from a PeerStatus message. Also converts values to other types if specified.
         * @function toObject
         * @memberof PeerStatus
         * @static
         * @param {PeerStatus} message PeerStatus
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        PeerStatus.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.connected = false;
                object.peerId = "";
                object.syncStatus = "";
                object.protocolStateHash = "";
                object.gitCommit = "";
                if ($util.Long) {
                    var long = new $util.Long(0, 0, false);
                    object.uptimeMinutes = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
                } else
                    object.uptimeMinutes = options.longs === String ? "0" : 0;
            }
            if (message.connected != null && message.hasOwnProperty("connected"))
                object.connected = message.connected;
            if (message.peerId != null && message.hasOwnProperty("peerId"))
                object.peerId = message.peerId;
            if (message.syncStatus != null && message.hasOwnProperty("syncStatus"))
                object.syncStatus = message.syncStatus;
            if (message.protocolStateHash != null && message.hasOwnProperty("protocolStateHash"))
                object.protocolStateHash = message.protocolStateHash;
            if (message.gitCommit != null && message.hasOwnProperty("gitCommit"))
                object.gitCommit = message.gitCommit;
            if (message.uptimeMinutes != null && message.hasOwnProperty("uptimeMinutes"))
                if (typeof message.uptimeMinutes === "number")
                    object.uptimeMinutes = options.longs === String ? String(message.uptimeMinutes) : message.uptimeMinutes;
                else
                    object.uptimeMinutes = options.longs === String ? $util.Long.prototype.toString.call(message.uptimeMinutes) : options.longs === Number ? new $util.LongBits(message.uptimeMinutes.low >>> 0, message.uptimeMinutes.high >>> 0).toNumber() : message.uptimeMinutes;
            return object;
        };
    
        /**
         * Converts this PeerStatus to JSON.
         * @function toJSON
         * @memberof PeerStatus
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        PeerStatus.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };
    
        return PeerStatus;
    })();

    return $root;
});
