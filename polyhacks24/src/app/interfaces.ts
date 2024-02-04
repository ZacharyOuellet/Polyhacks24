export interface Node {
    x: number;
    y: number;
}

export interface Edge {
    from_index: number;
    to_index: number;
}

export interface Graph {
    nodes: Node[];
    edges: Edge[];
}

export interface SolutionRequest {
    graph: Graph;
    driver_start: number;
    driver_end: number;
    passenger_start: number;
    passenger_end: number;
}

export interface SolutionResponse {
    driver_alone: number[];
    driver_alone_distance: number;
    driver_passenger: number[];
    driver_passenger_distance: number;
}