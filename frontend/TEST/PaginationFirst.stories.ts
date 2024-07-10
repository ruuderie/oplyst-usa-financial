import type { Meta, StoryObj } from '@storybook/vue3';

import PaginationFirst from '../components/ui/pagination/PaginationFirst.vue';

const meta = {
  title: 'PaginationFirst',
  component: PaginationFirst,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof PaginationFirst>;

export default meta;
type Story = StoryObj<typeof PaginationFirst>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};