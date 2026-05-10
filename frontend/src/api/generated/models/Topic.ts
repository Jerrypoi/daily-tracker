/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
export type Topic = {
    /**
     * Unique identifier for the topic (decimal string of an int64; sent as a string to avoid JS Number precision loss)
     */
    id: string;
    /**
     * Name of the topic (e.g., 'playing', 'working')
     */
    topic_name: string;
    /**
     * Topic display color in hex format (#RRGGBB)
     */
    display_color: string;
    /**
     * Timestamp when the topic was created
     */
    created_at: string;
    /**
     * Timestamp when the topic was last updated
     */
    updated_at: string;
    /**
     * ID of the parent topic (null for root-level topics)
     */
    parent_topic_id?: string;
};

