import { Component, OnInit, Input, AfterViewInit } from '@angular/core';
import { NodeIdObj } from 'src/app/interfaces';
import { NoeuComponent } from '../noeu/noeu.component';

import { Graph } from 'src/app/interfaces';
import * as d3 from 'd3';
import { GraphService } from 'src/app/service/graph.service';
//https://blog.logrocket.com/data-visualization-angular-d3-js/#setting-up-angular-d3

@Component({
  selector: 'app-svg-map',
  standalone: true,
  templateUrl: './svg-map.component.html',
  styleUrls: ['./svg-map.component.scss']
})

export class SvgMapComponent  {
    
    ratioX : number = 890-50;
    ratioY : number = 480-50;
    nodeId : number = -1;
    color: string[] = ['red','red','red']
    formData = {
      condDep: '',
      condArr: '',
      passDep: '',
      passArr: ''
    };
    constructor(private graphService: GraphService){
    }

    requestRedraw(){
        this.graphService.regenarateGraph().subscribe({next: (graph:Graph) => {
            console.log("redraw");
            this.graphService.graph = graph;
            this.draw();
        }});
    }

    draw(){
        console.log("draw");
        let i = 0;
        console.log(this.graphService.graph.nodes);

            const svg = d3.select("#map")
      .append('svg')
      .attr("preserveAspectRatio", "xMinYMin meet")
      .attr("viewBox", "0 0 960 500")
      .attr("transform",
        `translate(${50}, ${50})`);

      this.graphService.graph.edges.forEach(edge => {
        const source = this.graphService.graph.nodes[edge.from_index];
        const target = this.graphService.graph.nodes[edge.to_index];
        svg.append("line")
          .attr("x1", source.x*this.ratioX)
          .attr("y1", source.y*this.ratioY)
          .attr("x2", target.x*this.ratioX)
          .attr("y2", target.y*this.ratioY)
          .style("stroke", "black")
          .style("stroke-width", 2);
      });
      
      const self = this; // store reference to 'this' context
    svg.selectAll('circle')
      .data(this.graphService.graph.nodes)
      .enter()
      .append('circle')
      .attr('cx', d => d.x*this.ratioX)
      .attr('cy', d => d.y*this.ratioY)
      .attr('r', 10)
      .attr('id', d => d.id) 
      .style('fill', 'red')
      .on('click', function() {
        console.log(this.id);
        self.color[parseInt(this.id)] = 'blue';
        self.onDotClick(parseInt(this.id));
      });

      svg.selectAll('text')
      .data(this.graphService.graph.nodes)
      .enter()
      .append('text')
      .attr('x', d => d.x * this.ratioX)
      .attr('y', d => d.y * this.ratioY)
      .text(d => d.id)
      .attr('text-anchor', 'middle')
      .attr('dy', '0.3em') // Adjust vertical alignment
      .style('fill', 'white')
      .style('font-size', '12px');
    }

  

  onDotClick(nodeid: number): void {
    const svg = d3.select("#map")
      .append('svg')
      .attr("preserveAspectRatio", "xMinYMin meet")
      .attr("viewBox", "0 0 960 500")
      .attr("transform",
        `translate(${50}, ${50})`);

      this.graphService.graph.edges.forEach(edge =>{
        const source = this.graphService.graph.nodes[edge.from_index];
        const target = this.graphService.graph.nodes[edge.to_index];
        svg.append("line")
          .attr("x1", source.x*this.ratioX)
          .attr("y1", source.y*this.ratioY)
          .attr("x2", target.x*this.ratioX)
          .attr("y2", target.y*this.ratioY)
          .style("stroke", "black")
          .style("stroke-width", 2);
      });
      
      const self = this; // store reference to 'this' context
      svg.selectAll('circle')
      .data(this.graphService.graph.nodes)
      .enter()
      .append('circle')
      .attr('cx', d => d.x*this.ratioX)
      .attr('cy', d => d.y*this.ratioY)
      .attr('r', 10)
      .attr('id', d => d.id) 
      .style('fill', d => this.color[d.id])
      .on('click', function() {
        console.log(this.id);
        self.color[parseInt(this.id)] = 'blue';
        self.onDotClick(parseInt(this.id));
      });
  }
  soumettre(a:string,b:string,c:string,d:string){
    const cd = a;
    const ca = b;
    const pd = c;
    const pa = d;
    //send to server
    //generate
  }
}

