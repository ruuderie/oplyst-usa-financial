import type { Meta, StoryObj } from '@storybook/vue3';

import DropdownMenuSub from '../components/ui/dropdown-menu/DropdownMenuSub.vue';

const meta = {
  title: 'DropdownMenuSub',
  component: DropdownMenuSub,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof DropdownMenuSub>;

export default meta;
type Story = StoryObj<typeof DropdownMenuSub>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};