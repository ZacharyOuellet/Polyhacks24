import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
@Injectable({
  providedIn: 'root'
})
export class HttpManagerService {
  constructor(private http: HttpClient) {}

  getgrap() {
}

getSolution(graph: any, driverStart: any, driverEnd: any, passengerStart: any, passengerEnd: any) {
}
}
