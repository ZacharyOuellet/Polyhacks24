import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Node, Edge, Graph, SolutionRequest, SolutionResponse } from '../interfaces';


@Injectable({
  providedIn: 'root'
})
export class HttpManagerService {
  constructor(private http: HttpClient) {}

 async getgrap(){
    return await this.http.get('http://localhost:3000/graph');
}

async getSolution(graph: Graph, driverStart: number, driverEnd: number, passengerStart: number, passengerEnd: number) {
  let requestBody : SolutionRequest = {
    graph: graph,
    driver_start: driverStart,
    driver_end: driverEnd,
    passenger_start: passengerStart,
    passenger_end: passengerEnd
  }
  return await this.http.post('http://localhost:3000/solution', {requestBody});
}
}
