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
    
    $root.ConnectRequest = (function() {
    
        /**
         * Properties of a ConnectRequest.
         * @exports IConnectRequest
         * @interface IConnectRequest
         * @property {string|null} [address] ConnectRequest address
         */
    
        /**
         * Constructs a new ConnectRequest.
         * @exports ConnectRequest
         * @classdesc Represents a ConnectRequest.
         * @implements IConnectRequest
         * @constructor
         * @param {IConnectRequest=} [properties] Properties to set
         */
        function ConnectRequest(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }
    
        /**
         * ConnectRequest address.
         * @member {string} address
         * @memberof ConnectRequest
         * @instance
         */
        ConnectRequest.prototype.address = "";
    
        /**
         * Creates a new ConnectRequest instance using the specified properties.
         * @function create
         * @memberof ConnectRequest
         * @static
         * @param {IConnectRequest=} [properties] Properties to set
         * @returns {ConnectRequest} ConnectRequest instance
         */
        ConnectRequest.create = function create(properties) {
            return new ConnectRequest(properties);
        };
    
        /**
         * Encodes the specified ConnectRequest message. Does not implicitly {@link ConnectRequest.verify|verify} messages.
         * @function encode
         * @memberof ConnectRequest
         * @static
         * @param {IConnectRequest} message ConnectRequest message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        ConnectRequest.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.address != null && Object.hasOwnProperty.call(message, "address"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.address);
            return writer;
        };
    
        /**
         * Encodes the specified ConnectRequest message, length delimited. Does not implicitly {@link ConnectRequest.verify|verify} messages.
         * @function encodeDelimited
         * @memberof ConnectRequest
         * @static
         * @param {IConnectRequest} message ConnectRequest message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        ConnectRequest.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };
    
        /**
         * Decodes a ConnectRequest message from the specified reader or buffer.
         * @function decode
         * @memberof ConnectRequest
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {ConnectRequest} ConnectRequest
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        ConnectRequest.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.ConnectRequest();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.address = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };
    
        /**
         * Decodes a ConnectRequest message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof ConnectRequest
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {ConnectRequest} ConnectRequest
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        ConnectRequest.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };
    
        /**
         * Verifies a ConnectRequest message.
         * @function verify
         * @memberof ConnectRequest
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        ConnectRequest.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.address != null && message.hasOwnProperty("address"))
                if (!$util.isString(message.address))
                    return "address: string expected";
            return null;
        };
    
        /**
         * Creates a ConnectRequest message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof ConnectRequest
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {ConnectRequest} ConnectRequest
         */
        ConnectRequest.fromObject = function fromObject(object) {
            if (object instanceof $root.ConnectRequest)
                return object;
            var message = new $root.ConnectRequest();
            if (object.address != null)
                message.address = String(object.address);
            return message;
        };
    
        /**
         * Creates a plain object from a ConnectRequest message. Also converts values to other types if specified.
         * @function toObject
         * @memberof ConnectRequest
         * @static
         * @param {ConnectRequest} message ConnectRequest
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        ConnectRequest.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults)
                object.address = "";
            if (message.address != null && message.hasOwnProperty("address"))
                object.address = message.address;
            return object;
        };
    
        /**
         * Converts this ConnectRequest to JSON.
         * @function toJSON
         * @memberof ConnectRequest
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        ConnectRequest.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };
    
        return ConnectRequest;
    })();

    return $root;
});
