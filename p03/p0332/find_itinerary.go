package p0332

import (
	"sort"
)

const start = "JFK"

func findItinerary(tickets [][]string) []string {
	sort.Slice(tickets, func(i, j int) bool { return tickets[i][1] < tickets[j][1] })
	ticketsUsed := make([]bool, len(tickets))
	path := make([]string, len(tickets)+1)
	path[0] = start

	isFind := findNext(tickets, ticketsUsed, 1, path)
	if isFind {
		return path
	}
	return nil
}

func findNext(tickets [][]string, ticketsUsed []bool, loc int, path []string) (isFind bool) {
	if loc == len(path) {
		return true
	}
	prev := path[loc-1]
	for i, ticket := range tickets {
		if !ticketsUsed[i] && ticket[0] == prev {
			path[loc] = ticket[1]
			ticketsUsed[i] = true
			isFind = findNext(tickets, ticketsUsed, loc+1, path)
			if isFind {
				return true
			}
			ticketsUsed[i] = false
		}
	}
	return false
}
