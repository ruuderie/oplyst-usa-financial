import type { Meta, StoryObj } from '@storybook/vue3';

import PaginationEllipsis from '../components/ui/pagination/PaginationEllipsis.vue';

const meta = {
  title: 'PaginationEllipsis',
  component: PaginationEllipsis,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof PaginationEllipsis>;

export default meta;
type Story = StoryObj<typeof PaginationEllipsis>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};