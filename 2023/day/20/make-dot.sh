perl -le'print "digraph {"; while (<>) { /^([%&]?)(\w+) (-> .*)/; print "$2 [label=\"$1$2\"]; $2 $3;" } print "}"' input > input.dot
dot input.dot -Tpng -oinput.png

