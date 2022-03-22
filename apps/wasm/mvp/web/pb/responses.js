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
    
    $root.CommonResponse = (function() {
    
        /**
         * Properties of a CommonResponse.
         * @exports ICommonResponse
         * @interface ICommonResponse
         * @property {boolean|null} [success] CommonResponse success
         * @property {string|null} [errorMessage] CommonResponse errorMessage
         */
    
        /**
         * Constructs a new CommonResponse.
         * @exports CommonResponse
         * @classdesc Represents a CommonResponse.
         * @implements ICommonResponse
         * @constructor
         * @param {ICommonResponse=} [properties] Properties to set
         */
        function CommonResponse(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }
    
        /**
         * CommonResponse success.
         * @member {boolean} success
         * @memberof CommonResponse
         * @instance
         */
        CommonResponse.prototype.success = false;
    
        /**
         * CommonResponse errorMessage.
         * @member {string} errorMessage
         * @memberof CommonResponse
         * @instance
         */
        CommonResponse.prototype.errorMessage = "";
    
        /**
         * Creates a new CommonResponse instance using the specified properties.
         * @function create
         * @memberof CommonResponse
         * @static
         * @param {ICommonResponse=} [properties] Properties to set
         * @returns {CommonResponse} CommonResponse instance
         */
        CommonResponse.create = function create(properties) {
            return new CommonResponse(properties);
        };
    
        /**
         * Encodes the specified CommonResponse message. Does not implicitly {@link CommonResponse.verify|verify} messages.
         * @function encode
         * @memberof CommonResponse
         * @static
         * @param {ICommonResponse} message CommonResponse message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        CommonResponse.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.success != null && Object.hasOwnProperty.call(message, "success"))
                writer.uint32(/* id 1, wireType 0 =*/8).bool(message.success);
            if (message.errorMessage != null && Object.hasOwnProperty.call(message, "errorMessage"))
                writer.uint32(/* id 2, wireType 2 =*/18).string(message.errorMessage);
            return writer;
        };
    
        /**
         * Encodes the specified CommonResponse message, length delimited. Does not implicitly {@link CommonResponse.verify|verify} messages.
         * @function encodeDelimited
         * @memberof CommonResponse
         * @static
         * @param {ICommonResponse} message CommonResponse message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        CommonResponse.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };
    
        /**
         * Decodes a CommonResponse message from the specified reader or buffer.
         * @function decode
         * @memberof CommonResponse
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {CommonResponse} CommonResponse
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        CommonResponse.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.CommonResponse();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.success = reader.bool();
                    break;
                case 2:
                    message.errorMessage = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };
    
        /**
         * Decodes a CommonResponse message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof CommonResponse
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {CommonResponse} CommonResponse
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        CommonResponse.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };
    
        /**
         * Verifies a CommonResponse message.
         * @function verify
         * @memberof CommonResponse
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        CommonResponse.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.success != null && message.hasOwnProperty("success"))
                if (typeof message.success !== "boolean")
                    return "success: boolean expected";
            if (message.errorMessage != null && message.hasOwnProperty("errorMessage"))
                if (!$util.isString(message.errorMessage))
                    return "errorMessage: string expected";
            return null;
        };
    
        /**
         * Creates a CommonResponse message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof CommonResponse
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {CommonResponse} CommonResponse
         */
        CommonResponse.fromObject = function fromObject(object) {
            if (object instanceof $root.CommonResponse)
                return object;
            var message = new $root.CommonResponse();
            if (object.success != null)
                message.success = Boolean(object.success);
            if (object.errorMessage != null)
                message.errorMessage = String(object.errorMessage);
            return message;
        };
    
        /**
         * Creates a plain object from a CommonResponse message. Also converts values to other types if specified.
         * @function toObject
         * @memberof CommonResponse
         * @static
         * @param {CommonResponse} message CommonResponse
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        CommonResponse.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.success = false;
                object.errorMessage = "";
            }
            if (message.success != null && message.hasOwnProperty("success"))
                object.success = message.success;
            if (message.errorMessage != null && message.hasOwnProperty("errorMessage"))
                object.errorMessage = message.errorMessage;
            return object;
        };
    
        /**
         * Converts this CommonResponse to JSON.
         * @function toJSON
         * @memberof CommonResponse
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        CommonResponse.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };
    
        return CommonResponse;
    })();

    return $root;
});
