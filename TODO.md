#TODO 
this document is to write down some of the goals of the service and to write down ideas for the implementation.

## DATA
this is a little tricky, either we can try to source this from gamma the account site but it does not have the information about all of the posts as some are labled "member" and not a post name on the granularity I would like to track. this option will most likely be uppdated for a long time in the future, at the group level this is useful. one option might be that when you sign in after gaining a new group it prompts you to select your parent from the previous year. gamma has insufficient information from earlier than 2018 depending on what society or committee we look at.

the other option could be that every time a new committy and society is added someone manualy has to put in the members and their parent.

old data will most likely have to be put in by hand. 

### data structure
the current idea is to divide the data into societies where a group has a:
- year
- list of members which is a map of cid and a tuple of ("post id", and "post name")
the idea is that we pair a direct patet with the id and are not dependent on different years switching up the names of the posts.

users which have:
- cid
- nick
- list of groups, from which we could see a list of postnames

society which has:
- name 
- list of groups


## view 
I had the idea that there should be different ways to veiw the data. 

### user view 
you can see each group that a user is a member of, this should just be a prettier version of the info on gamma.

### user relations view 
you can search up two or more users and see all the relations between them visualized in some pretty way to show type of relation and what society.

### soceity view 
you can see the family tree of a society so the year to year direct paternity.
it might be fun to here have the ability to also see relations from other societies visualized over the tree so it would be like all of the user relations veiws over the society view.

### home page veiw
I think it would be fun for the home page to have a graph visualizing how peple move through the different societies so we have bubbles for each society and for each pair  of groups where at least one person  has been in both groups we make a line between those grops and the more people the thicker the line.
this could also be fun to filter by year ranges.

### the cluster fuck 
this is a view where we have all of the users who have been active in any society and their relations to each other. so user relations view but for everyone. should perhaps have an option to filter by year range, or by number of relations unique relations for two users to show up. this view would probably be too messy if we try to treat all patets as a relation and should be limited to only parents and siblings.


## relations
there are different categories of relations:
- siblings - the people who are in the same society in the same year, aka the same group.
- parent - the person who had the same post the year before, for earlier year you get grandparent
- patet - anyone who was in the same society before you. 
- child - the person who has your post the year after, or is in the same society after you, or for later years grand child. 
