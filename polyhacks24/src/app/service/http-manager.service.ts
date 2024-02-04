import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
@Injectable({
  providedIn: 'root'
})

export interface solutionResponse{
  driver_alone: Array<number>;
   driver_alone_distance: number;
    driver_passenger: Array<number>;
     driver_passenger_distance: number;
}

export class HttpManagerService {
  constructor(private http: HttpClient) {}

 async getgrap() {
    return await this.http.get('http://localhost:3000/graph');
}

async getSolution(graph: any, driverStart: any, driverEnd: any, passengerStart: any, passengerEnd: any) {
  let requestBody = {
    graph: graph,
    driverStart: driverStart,
    driverEnd: driverEnd,
    passengerStart: passengerStart,
    passengerEnd: passengerEnd
  }
  return await this.http.post('http://localhost:3000/solution', {requestBody});
}
}
