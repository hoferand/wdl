/**
 * Location
 *
 * @typedef {{line: number, column: number}} Location
 */

/**
 * Span
 *
 * @typedef {{start: Location, end: Location}} Span
 */

/**
 * MarkerSeverity
 *
 * @typedef {"Hint"|"Info"|"Warning"|"Error"} MarkerSeverity
 */

/**
 * Marker
 *
 * @typedef {{severity: MarkerSeverity, message: string, span: Span}} Marker
 */

/**
 * ResponseCallback
 *
 * @callback ResponseCallback
 * @param {"Done"|"NoStationLeft"} status
 * @returns {void}
 */

/**
 * LogLevel
 *
 * @typedef {"Trace"|"Debug"|"Info"|"Warn"|"Error"} LogLevel
 */

/**
 * Log
 *
 * `msg` is not html escaped
 * `span_str` is html escaped
 *
 * @typedef {{level: LogLevel, msg: string, user?: boolean, span?: Span, span_str?: string}} Log
 */

/**
 * Position
 *
 * @typedef {{span: Span, span_str: string}} Position
 */

/**
 * WdlError
 *
 * @typedef {{title: string, pos?: Position}} WdlError
 */
