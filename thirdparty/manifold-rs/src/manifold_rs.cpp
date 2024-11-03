#include "manifold_rs.h"

#include "manifold.h"

namespace manifold_rs
{
    Manifold::Manifold() : manifold(std::make_unique<::manifold::Manifold>()) {}
    Manifold::Manifold(::manifold::Manifold &&manifold) : manifold(std::make_unique<::manifold::Manifold>(std::move(manifold))) {}
    Manifold::~Manifold() {}

    std::unique_ptr<Manifold> sphere(double radius, uint32_t circular_segments)
    {
        return std::make_unique<Manifold>(::manifold::Manifold::Sphere(radius, circular_segments));
    }

    std::unique_ptr<Manifold> cube(double x_size, double y_size, double z_size)
    {
        return std::make_unique<Manifold>(::manifold::Manifold::Cube({x_size, y_size, z_size}));
    }

    std::unique_ptr<Manifold> cylinder(double radius_low, double radius_height, double height, uint32_t circular_segments)
    {
        return std::make_unique<Manifold>(::manifold::Manifold::Cylinder(height, radius_low, radius_height, circular_segments));
    }

    std::unique_ptr<Manifold> union_(const Manifold &a, const Manifold &b)
    {
        return std::make_unique<Manifold>(a.manifold->Boolean(*b.manifold, ::manifold::OpType::Add));
    }

    std::unique_ptr<Manifold> intersection(const Manifold &a, const Manifold &b)
    {
        return std::make_unique<Manifold>(a.manifold->Boolean(*b.manifold, ::manifold::OpType::Intersect));
    }

    std::unique_ptr<Manifold> difference(const Manifold &a, const Manifold &b)
    {
        return std::make_unique<Manifold>(a.manifold->Boolean(*b.manifold, ::manifold::OpType::Subtract));
    }

    Mesh::Mesh() : mesh(std::make_unique<::manifold::Mesh>()) {}

    Mesh::Mesh(::manifold::Mesh &&mesh) : mesh(std::make_unique<::manifold::Mesh>(std::move(mesh))) {}

    Mesh::~Mesh() {}

    std::unique_ptr<std::vector<float>> Mesh::vertices() const
    {
        std::vector<float> vertices;
        vertices.reserve(mesh->vertPos.size() * 6);
        assert(mesh->vertPos.size() == mesh->vertNormal.size());
        for (size_t i = 0; i < mesh->vertPos.size(); i++)
        {
            vertices.push_back(mesh->vertPos[i].x);
            vertices.push_back(mesh->vertPos[i].y);
            vertices.push_back(mesh->vertPos[i].z);
            vertices.push_back(mesh->vertNormal[i].x);
            vertices.push_back(mesh->vertNormal[i].y);
            vertices.push_back(mesh->vertNormal[i].z);
        }
        return std::make_unique<std::vector<float>>(vertices);
    }

    std::unique_ptr<std::vector<uint32_t>> Mesh::indices() const
    {
        std::vector<uint32_t> indices;
        indices.reserve(mesh->triVerts.size() * 3);
        for (size_t i = 0; i < mesh->triVerts.size(); i++)
        {
            indices.push_back(mesh->triVerts[i].x);
            indices.push_back(mesh->triVerts[i].y);
            indices.push_back(mesh->triVerts[i].z);
        }
        return std::make_unique<std::vector<uint32_t>>(indices);
    }

    std::unique_ptr<Mesh> mesh_from_manifold(const Manifold &manifold)
    {
        return std::make_unique<Mesh>(manifold.manifold->GetMesh());
    }

    std::unique_ptr<Manifold> manifold_from_mesh(const Mesh &mesh)
    {
        return std::make_unique<Manifold>(::manifold::Manifold(*mesh.mesh));
    }

    std::unique_ptr<Mesh> mesh_from_vertices(
        rust::Slice<const float> vertices, 
        rust::Slice<const uint32_t> indices)
    {
        assert(vertices.size() % 6 == 0);
        assert(indices.size() % 3 == 0);
        ::manifold::Mesh mesh;
        mesh.vertPos.reserve(vertices.size() / 6);
        mesh.vertNormal.reserve(vertices.size() / 6);
        for (size_t i = 0; i < vertices.size(); i += 6)
        {
            mesh.vertPos.push_back({vertices[i], vertices[i + 1], vertices[i + 2]});
            mesh.vertNormal.push_back({vertices[i + 3], vertices[i + 4], vertices[i + 5]});
        }
        mesh.triVerts.reserve(indices.size() / 3);
        for (size_t i = 0; i < indices.size(); i += 3)
        {
            mesh.triVerts.push_back({indices[i], indices[i + 1], indices[i + 2]});
        }
        return std::make_unique<Mesh>(std::move(mesh));
    }

} // namespace manifold_rs
