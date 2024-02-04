import { Injectable } from '@angular/core';
import { HttpManagerService } from './http-manager.service';
import { Graph, SolutionRequest, SolutionResponse } from '../interfaces';

@Injectable({
    providedIn: 'root'
})
export class GraphService {
    public graph: Graph;
    public driverStart: number;
    public driverEnd: number;
    public passengerStart: number;
    public passengerEnd: number;
    async regenarateGraph() {
        this.graph = await this.httpManager.getGraph() as unknown as Graph;
    }

    constructor(private httpManager: HttpManagerService) { 
        this.graph = {
            nodes: [],
            edges: []
        };
        this.driverStart = 0;
        this.driverEnd = 0;
        this.passengerStart = 0;
        this.passengerEnd = 0;
        this.regenarateGraph();
    }


    async getSolution(){
        if (this.graph.nodes.length == 0) {
            return {
                driver_alone: [],
                driver_alone_distance: 0,
                driver_passenger: [],
                driver_passenger_distance: 0
            };
        }
        return await this.httpManager.getSolution(this.graph, this.driverStart, this.driverEnd, this.passengerStart, this.passengerEnd);
    }
}
