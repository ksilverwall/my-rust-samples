pragma solidity ^0.8.0;

contract SocialMedia {
    struct Post {
        string content;
        uint256 timestamp;
        bool deleted;
    }

    mapping (address => Post[]) private posts;

    function createPost(string memory content) public {
        Post memory newPost = Post({
            content: content,
            timestamp: block.timestamp,
            deleted: false
        });
        posts[msg.sender].push(newPost);
    }

    function getPosts(address account) public view returns (Post[] memory) {
        Post[] memory result = new Post[](posts[account].length);
        uint256 index = 0;
        for (uint256 i = 0; i < posts[account].length; i++) {
            if (!posts[account][i].deleted) {
                result[index] = posts[account][i];
                index++;
            }
        }
        return result;
    }

    function deletePost(uint256 index) public {
        require(index < posts[msg.sender].length, "Invalid index.");
        posts[msg.sender][index].deleted = true;
    }
}
