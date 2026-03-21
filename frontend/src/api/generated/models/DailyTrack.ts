/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
export type DailyTrack = {
    /**
     * Unique identifier for the daily track record
     */
    id: number;
    /**
     * Start time of the 30-minute period (must be at :00 or :30)
     */
    start_time: string;
    /**
     * Timestamp when the record was created
     */
    created_at: string;
    /**
     * Timestamp when the record was last updated
     */
    updated_at: string;
    /**
     * ID of the associated topic
     */
    topic_id: number;
    /**
     * Optional notes or comments for this time period
     */
    comment?: string;
};

