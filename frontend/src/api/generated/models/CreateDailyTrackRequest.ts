/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
export type CreateDailyTrackRequest = {
    /**
     * Start time of the 30-minute period (must be at :00 or :30)
     */
    start_time: string;
    /**
     * ID of the topic for this time period
     */
    topic_id: number;
    /**
     * Optional notes or comments
     */
    comment?: string;
};

