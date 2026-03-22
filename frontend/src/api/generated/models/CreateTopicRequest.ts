/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
export type CreateTopicRequest = {
    /**
     * Name of the topic to create
     */
    topic_name: string;
    /**
     * Optional parent topic ID for hierarchical organization
     */
    parent_topic_id?: number;
    /**
     * Optional topic display color in hex format (#RRGGBB). Defaults to #3b82f6
     */
    display_color?: string;
};

