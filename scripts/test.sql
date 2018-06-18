Select i.name, a.quantity, 
a.buyout / 10000 AS buyout
From auction a
INNER JOIN item i on a.item = i.id
WHERE i.name = 'Aethril'
ORDER BY a.buyout / a.quantity