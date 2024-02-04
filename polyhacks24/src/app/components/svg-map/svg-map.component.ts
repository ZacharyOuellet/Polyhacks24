import { Component, OnInit, Input } from '@angular/core';
import { NoeuComponent } from '../noeu/noeu.component';
import * as d3 from 'd3';
//https://blog.logrocket.com/data-visualization-angular-d3-js/#setting-up-angular-d3

interface Node {
  id: number;
  x: number;
  y: number;
}

@Component({
  selector: 'app-svg-map',
  standalone: true,
  templateUrl: './svg-map.component.html',
  styleUrls: ['./svg-map.component.scss']
})

export class SvgMapComponent implements OnInit {
    @Input() nodes;
    @Input() links : number[][];
    ratioX : number = 890-50;
    ratioY : number = 480-50;
    nodeId : number = -1;
    constructor(){
      this.nodes = [
        { id: 0, x:0.7, y: 0.5},
        { id: 1, x: 0.1, y: 0.8 },
        { id: 2, x: 0.6, y: 0.3 }
      ];
      this.links = [[0,1],[1,2],[2,0]]
    }

    ngOnInit(): void {
      const svg = d3.select("#map")
      .append('svg')
      .attr("preserveAspectRatio", "xMinYMin meet")
      .attr("viewBox", "0 0 960 500")
      .attr("transform",
        `translate(${50}, ${50})`);

      this.links.forEach(link => {
        const source = this.nodes[link[0]];
        const target = this.nodes[link[1]];
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
      .data(this.nodes)
      .enter()
      .append('circle')
      .attr('cx', d => d.x*this.ratioX)
      .attr('cy', d => d.y*this.ratioY)
      .attr('r', 10)
      .attr('id', d => d.id) 
      .style('fill', 'red')
      .on('click', function() {
        console.log(this.id);
      });
  }

  onDotClick(nodeid: number): void {
    console.log(`Clicked node ${nodeid}`);
    // Handle click event here
  }
}

